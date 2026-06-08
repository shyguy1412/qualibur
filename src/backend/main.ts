const pagePattern = new URLPattern({ pathname: '(.*?)/post.json' });

export const serve = () =>
    Deno.serve({ port: 8080 }, (req) => {
        const match = pagePattern.exec(req.url);

        if (match) {
            const path = match.pathname.input.replace(/post\.json$/, '');
            return Response.json({ path, hello: 'world', time: Date.now() }, {
                headers: { 'Content-Type': 'application/json' },
            });
        }

        return Response.json({ hello: 'world' });
    });
