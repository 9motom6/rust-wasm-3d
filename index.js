const rust = import("./pkg/index");

// rust.then(m => m.say_hello_from_rust())
//     .catch(console.log);

const canvas = document.getElementById("rustCanvas");
const gl = canvas.getContext("webgl", {antialias: true})

function checkWindowResize() {
    if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height = window.innerHeight;

        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;

        gl.viewport(0, 0, window.innerWidth, window.innerHeight);
    }
}

rust.then(m => {
    if (!gl) {
        alert("Failed to initialize WebGL");
        return;
    }

    const FPS_THROTTLE = 1000.0 / 60.0; // sets 60 FPS
    let lastDrawTime = -1; // in ms
    const graphicsClient = new m.GraphicsClient();
    const initialTime = Date.now();
    function render() {
        window.requestAnimationFrame(render);

        const currentTime = Date.now();

        if (currentTime >= lastDrawTime + FPS_THROTTLE) {
            lastDrawTime = currentTime;
            checkWindowResize();
        }
        let elapsedTime = currentTime - initialTime;

        graphicsClient.update(elapsedTime, window.innerHeight, window.innerWidth);
        graphicsClient.render();
    }

    render();
})