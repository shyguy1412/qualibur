import { Metafile } from 'esbuild';
import { dirname, relative, resolve } from 'node:path';
import { renderToString } from 'preact-render-to-string';
import { h } from 'preact';
import { Index, post } from './frontend/base.tsx';

const meta = JSON.parse(
    await Deno.readTextFile(import.meta.dirname! + '/meta.json'),
) as Metafile;

const files = Object.keys(meta.outputs).filter((f) =>
    !f.startsWith('dist/__chunk') && f.endsWith('.js')
);

for (const file of files) {
    const path = resolve(file);
    const { default: Page } = await import(path);
    if (!Page) {
        continue;
    }

    const postPath = relative(import.meta.dirname!, dirname(path) + '/post.json');

    console.log(path, postPath);

    const data = await fetch('http://localhost:8080/' + postPath).then((r) => r.json());
    post.value = data;

    const staticSite = renderToString(h(Index, { initialPage: Page }));
    const dir = dirname(resolve(import.meta.dirname!, path));

    await Deno.writeTextFile(dir + '/index.html', staticSite);
}
