import App from "./App.svelte";

function disableMenu() {
    if (window.location.hostname !== "tauri.localhost") {
        return;
    }

    document.addEventListener(
        "contextmenu",
        (e) => {
            e.preventDefault();
            return false;
        },
        { capture: true }
    );

    document.addEventListener(
        "selectstart",
        (e) => {
            e.preventDefault();
            return false;
        },
        { capture: true }
    );
}

disableMenu();

const app = new App({
    target: document.body,
});

export default app;
