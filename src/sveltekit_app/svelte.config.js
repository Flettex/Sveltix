import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: [
		{
			async markup({ content, filename }) {
				let newContent = content;
				console.log("\nFILE NAME: ", filename, "\n")
				// Don't use /g because we only need one match
				const pattern = /export\s+let\s+(\w+)\s*:\s*PageData/;
		
				const matched = pattern.exec(content);
		
				if (matched) {
					// matched[1] is he variable name
					const usagePattern = new RegExp(
						`\\{(\\w+\\()?${matched[1]}(\\.([\\w$]+))?(\\))?|(\\w+\\()?(${matched[1]}\\.([\\w$]+))(\\))`,
						"g"
					);
					let match;
					while ((match = usagePattern.exec(content))) {
						const usage = match[0];
						const attribute = match[1] || match[2];
						console.log(`Found usage of ${usage}, with attribute ${attribute}`);
						newContent = newContent.replace(match[0], `{\`{{SSR_DATA${attribute}}}\``);
						// Do something with the attribute
					}
				}
				console.log(newContent)
				return { code: newContent };
			},
		},
		vitePreprocess()
	],

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter()
	}
};

export default config;
