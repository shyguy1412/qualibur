import { h } from 'preact';
import { useEffect, useState } from 'preact/hooks';
import style from './HelloWorld.module.css';

export default function () {
    const [time, setTime] = useState(Date.now());

    useEffect(() => {
        const interval = setInterval(() => setTime(Date.now()), 0);
        return () => clearInterval(interval);
    }, [setTime]);

    return (
        <div class={style.test}>
            <h1>Test</h1>
            <p>
                Lorem ipsum dolor sit amet consectetur, adipisicing elit. Quo nesciunt
                quis vitae quibusdam error saepe consectetur, fugit minus quae veritatis
                quam explicabo magni inventore aliquid enim earum illum magnam
                repudiandae?
            </p>
            <a href='/test'>TEST</a>
            <br />
            <span>{time}</span>
        </div>
    );
}
