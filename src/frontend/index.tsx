import './index.css';
import 'normalize.css';

import { Fragment, h, render } from 'preact';
import { signal, useSignal, useSignalEffect } from '@preact/signals';

const formatUrlPath = (path: string) => `/pages${path.replace(/[\/]$/, '')}`;

const page = signal(formatUrlPath(window.location.pathname));
function Index() {
    const module = useSignal();

    useSignalEffect(() => {
        import(page.value + '/index.js')
            .then((mod) => module.value = mod.default);
    });

    const Page = () => {
        if (module.value) {
            return h(module.value, {});
        }

        return 'loading';
    };

    const Style = () => <link rel='stylesheet' href={page.value + '/index.css'} />;

    return <>
        <Page></Page>
        <Style></Style>
    </>;
}

render(<Index></Index>, document.body);

navigation.addEventListener('navigate', (event) => {
    const url = new URL(event.destination.url);

    if (url.host != window.location.host) {
        return;
    }

    page.value = formatUrlPath(url.pathname);

    event.intercept();
});
