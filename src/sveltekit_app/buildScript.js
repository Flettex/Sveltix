import fs from 'node:fs';
import { build } from 'vite';

try {
	await build();
	console.log('1st Build complete!');
} catch (error) {
	console.error(error);
}

fs.cpSync('./build', './build1', { recursive: true });

process.env.sveltixPreprocessDisable = true;

try {
	await build();
	console.log('2nd Build complete!');
} catch (error) {
	console.error(error);
}

const files = fs.readdirSync('./build1');

const htmlFiles = files.filter((file) => file.endsWith('.html'));

htmlFiles.forEach((file) => {
	const srcFile = fs.readFileSync(`./build1/${file}`, 'utf-8');
	const destPath = `./build/${file}`;
	const destFile = fs.readFileSync(`./build/${file}`, 'utf-8');

	const BODY_TAG_REGEX = /<body.+>.+<\/body>/gs;
	const SCRIPT_TAG_REGEX = /<script>.+<\/script>/gs;

	const srcFileBodyTag = BODY_TAG_REGEX.exec(srcFile)[0];

	const destFileScriptTag = SCRIPT_TAG_REGEX.exec(destFile);

	const scriptTagsToInject = destFileScriptTag
		.map((s) =>
			s.replace(
				/const\s+data\s*=\s*\[[^,\]]*?,\s*(.*?)(?:,\s*\S*?)?\s*\];/g,
				`const data = [null,{"type":"data","data": {{ SSR_DATA | safe }},"uses":{}}];`
			)
		)
		.join('');

	const newDestFile = destFile.replace(
		BODY_TAG_REGEX,
		srcFileBodyTag.replace(SCRIPT_TAG_REGEX, scriptTagsToInject)
	);

    console.log(newDestFile)

	fs.writeFileSync(destPath, newDestFile);
});

fs.rmSync("./build1", { recursive: true, force: true });

delete process.env.sveltixPreprocessDisable;
