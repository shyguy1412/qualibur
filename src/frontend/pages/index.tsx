import { h } from 'preact';
import HelloWorld from '@/frontend/components/HelloWorld';

// const post = document ?
//     await import(eval('"data:post"'), {
//         with: { type: 'json' },
//     }) :
//     {};

export default function () {
    return <HelloWorld></HelloWorld>;
}
