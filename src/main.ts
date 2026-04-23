import { context, Plugin } from 'esbuild';

const reloadPlugin: Plugin = {
    name: 'reloadPlugin',
    setup(pluginBuild) {
        pluginBuild.onLoad({ filter: /.*\.html$/ }, async (opts) => {
            const file = await Deno.readTextFile(opts.path);
            return {
                contents: file.replace(
                    '</head>',
                    "    <script>{const ev = new EventSource('http://localhost:3001/esbuild'); ev.addEventListener('change', () => (ev.close(),location.reload()) )}</script>\n</head>",
                ),
                loader: 'copy',
            };
        });
    },
};

const createContext = async () =>
    await context({
        entryPoints: [
            './src/frontend/index.tsx',
            './src/frontend/index.html',
            './src/frontend/pages/**/*.tsx',
        ],
        loader: { '.html': 'copy', '.woff2': 'copy' },
        plugins: [reloadPlugin],
        outbase: './src/frontend',
        outdir: './dist',
        bundle: true,
        format: 'esm',
        platform: 'browser',
        alias: {
            react: 'preact/compat',
            'react-dom': 'preact/compat',
            'react-reconciler': 'preact-reconciler',
        },
        // minify: true,
        sourcemap: true,
        chunkNames: '/pages/__chunks/chunk-[hash]',
        splitting: true,
        logLevel: 'info',
    });

const ctx = await createContext();
ctx.serve({ host: 'localhost', port: 3001, cors: { origin: 'http://localhost:3000' } });
ctx.watch();
