export const sveltixPreprocess = () => {
	return {
		async markup({ content, filename }) {
			if (process.env.sveltixPreprocessDisable) {
				return { code: content };
			}

			let newContent = content;
			// console.log('\nFILE NAME: ', filename, '\n');
			// Don't use /g because we only need one match
			const pattern = /export\s+(let|const)\s+(\w+)\s*:\s*PageData/;

			const matched = pattern.exec(content);

			if (matched) {
				// matched[1] is he variable name
				const usagePattern = new RegExp(
					`\\{(\\w+\\()?${matched[1]}(\\.([\\w$]+))?(\\))?|(\\w+\\()?(${matched[1]}\\.([\\w$]+))(\\))`,
					'g'
				);
				let match;
				while ((match = usagePattern.exec(content))) {
					// const usage = match[0];
					const attribute = match[1] || match[2];
					// console.log(`Found usage of ${usage}, with attribute ${attribute}`);
					newContent = newContent.replace(match[0], `{\`{{SSR_DATA${attribute}}}\``);
					// Do something with the attribute
				}
			}
			// console.log(newContent);
			return { code: newContent };
		}
	};
};
