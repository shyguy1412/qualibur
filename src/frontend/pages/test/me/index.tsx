import { h } from 'preact';
import { useEffect, useState } from 'preact/hooks';
import style from '../test.module.css';

export default function () {
    const [time, setTime] = useState(Date.now());

    useEffect(() => {
        const interval = setInterval(() => setTime(Date.now()), 1000);
        return () => clearInterval(interval);
    }, [setTime]);

    return (
        <div class={style.test}>
            <h1>TestRouted</h1>
            <p>
                Lorem ipsum dolor sit amet consectetur, adipisicing elit. Quo nesciunt
                quis vitae quibusdam error saepe consectetur, fugit minus quae veritatis
                quam explicabo magni inventore aliquid enim earum illum magnam
                repudiandae?
            </p>
            <a href='/'>HOME</a>
            <br />
            <span>{time}</span>
        </div>
    );
}
