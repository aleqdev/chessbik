export function write_clip(str) {
  ( async () => {
    console.log(await navigator.permissions.query({ name: 'clipboard-write' }));
    console.log(await navigator.clipboard.writeText(str));
  })()
}