import { createContext, Fragment, h, hydrate } from 'preact';
import { signal, useSignal, useSignalEffect } from '@preact/signals';
import { createRootFragment } from 'preact-root-fragment';
import initialPost from 'post.json' with { type: 'json' };

const navInterceptController = new AbortController();

if (globalThis.document) {
    const ev = new EventSource('http://localhost:3001/esbuild');
    ev.addEventListener('change', () => {
        ev.close();
        navInterceptController.abort();
        location.reload();
    });
    console.log('EVENT SOURCE CREATED');
}

const formatUrlPath = (path: string) => `${path.replace(/[\/]$/, '')}`;
const initialPath = formatUrlPath(globalThis.location?.pathname ?? '');

const page = signal(initialPath);
export const post = signal(initialPost);

export function Index({ initialPage }: any) {
    const module = useSignal(initialPage);

    useSignalEffect(() => {
        if (!document) {
            return;
        }
        import(page.value + '/index.js')
            .then((mod) => module.value = mod.default);
        import(page.value + '/post.json', { with: { type: 'json' } })
            .then((mod) => post.value = mod.default);
    });

    const Page = () => {
        if (module.value) {
            return h(module.value, {});
        }

        return 'loading';
    };

    const Style = () => <link rel='stylesheet' href={page.value + '/index.css'} />;

    const importmap = {
        imports: {
            'post.json': './post.json',
        },
    };

    const inner = JSON.stringify(importmap);

    return <html>
        <head>
            <link rel='stylesheet' href='/global.css' />
            <script type='importmap' dangerouslySetInnerHTML={{ __html: inner }} />
            <script type='module' src='./index.js' defer />
            <title>Qualibur</title>
        </head>

        <body>
            <Page />
        </body>
        <Style />
    </html>;
}

if (globalThis.document) {
    const root = createRootFragment(document.documentElement, document.documentElement);

    hydrate(<Index></Index>, root);

    navigation.addEventListener('navigate', (event) => {
        const url = new URL(event.destination.url);

        if (url.host != window.location.host) {
            return;
        }

        page.value = formatUrlPath(url.pathname);

        event.intercept();
    }, { signal: navInterceptController.signal });
}
