const rust = import("./pkg/wasm_js13k");
const canvas = document.getElementById("rustCanvas");
const gl = canvas.getContext("webgl", {antialias: true})


rust.then(m => {
    if(!gl){
        alert("Failed to initialize WebGL");
        return;
    }
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

    const FPS_THROTTLE = 1000.0 / 30.0;
    const client = new m.ClaraClient();
    const initialTime = Date.now();
    var lastDrawTime = -1;

    function render(){
        window.requestAnimationFrame(render);
        const currTime = Date.now();

        if(currTime >= lastDrawTime + FPS_THROTTLE)
        {
            lastDrawTime = currTime;

            if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }
            
            let elapsedTime =  currTime - initialTime;
            client.update(elapsedTime, window.innerHeight, window.innerWidth);
            client.render();
        }
    }
    render();
})
.catch(console.error);