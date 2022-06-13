export function read_clip() {
return ( async () => {
    console.log(await navigator.permissions.query({ name: 'clipboard-read' }));
    return await navigator.clipboard.readText();
})()
}