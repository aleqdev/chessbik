export function disable_ctx_menu() {
    document.addEventListener('contextmenu', event => event.preventDefault());
}