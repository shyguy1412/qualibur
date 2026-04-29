import { context, Plugin } from 'esbuild';
import { resolve } from 'node:path';
import { serve } from './backend/main.ts';

const ssgPlugin: Plugin = {
    name: 'ssgPlugin',
    setup(pluginBuild) {
        pluginBuild.onResolve({ filter: /^(node):/ }, () => ({ external: true }));
        pluginBuild.onResolve(
            { filter: /^(data):/ },
            (opts) =>
                opts.with.type == 'json' ?
                    { external: true } :
                    { path: opts.path, namespace: 'data' },
        );

        pluginBuild.onResolve({ filter: /@static-site-generator/ }, () => {
            return {
                path: './src/frontend/pages/ssg.ts',
                namespace: '@static-site-generator',
            };
        });

        pluginBuild.onLoad(
            { filter: /.*/, namespace: 'data' },
            (opts) => ({
                contents:
                    `import data from "${opts.path}" with { type: "json" };export default data;`,
            }),
        );

        pluginBuild.onLoad(
            { filter: /.*/, namespace: '@static-site-generator' },
            async () => {
                return {
                    contents: await Deno.readTextFile('./src/ssg.ts'),
                    resolveDir: './src',
                    loader: 'ts',
                    watchFiles: ['./src/ssg.ts'],
                };
            },
        );

        pluginBuild.onEnd(async (opts) => {
            await Deno.writeTextFile(
                pluginBuild.initialOptions.outdir + '/meta.json',
                JSON.stringify(opts.metafile!),
            );

            const ssgPath = `${
                resolve(pluginBuild.initialOptions.outdir!)
            }/@static-site-generator.js`;

            await Deno.spawn(
                `deno`,
                { args: ['-A', ssgPath] },
            ).status.catch((e) => console.error(e));
        });
    },
};

const createContext = async () =>
    await context({
        entryPoints: [
            './src/frontend/pages/**/*.tsx',
            './src/frontend/pages/global.css',
            './src/frontend/pages/@static-site-generator',
        ],
        loader: { '.html': 'file', '.woff2': 'copy' },
        inject: [
            '@/ssg-shim',
            'preact',
            'preact/hooks',
            'preact/compat',
            '@/frontend/base',
        ],
        plugins: [
            ssgPlugin,
        ],
        outbase: './src/frontend/pages',
        outdir: './dist',
        bundle: true,
        metafile: true,
        format: 'esm',
        platform: 'browser',
        external: ['post.json'],
        alias: {
            react: 'preact/compat',
            'react-dom': 'preact/compat',
            'react-reconciler': 'preact-reconciler',
        },
        sourcemap: 'inline',
        splitting: true,
        chunkNames: '/__chunks/chunk-[hash]',
        logLevel: 'info',
    });

const ctx = await createContext();
// ctx.serve({
//     host: 'localhost',
//     port: 3000,
//     servedir: './dist',
// });
ctx.watch();

serve();
