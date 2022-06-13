export function read_clip() {
( async () => {
    console.log(await navigator.permissions.query({ name: 'clipboard-read' }));
    console.log(await navigator.clipboard.readText().then(str => {return str}));
})()
}