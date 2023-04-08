import * as wasm from "ex036";
import {memory} from "ex036/ex036_bg.wasm"

const CELL_SIZE = 5;
const GRID_COLOR = "#a19f9f";
const DEAD_COLOR= "#000000";
const ALIVE_COLOR = "#d6d4d4";
const universe = wasm.Universe.new();
let width = universe.width();
let height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const drawCells = () => {
    const delta = universe.delta();
    universe.clear_delta();
    ctx.beginPath();

    ctx.fillStyle = ALIVE_COLOR;
    for(let idx = 0; idx < delta.length; idx++){
        let row = Math.floor(delta[idx] / width)
        let col = delta[idx] % width
        if(universe.check_cell(delta[idx]) == wasm.Cell.Alive){
            ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
        }
    }

    ctx.fillStyle = DEAD_COLOR;
    for(let idx = 0; idx < delta.length; idx++){
        let row = Math.floor(delta[idx] / width)
        let col = delta[idx] % width
        if(universe.check_cell(delta[idx]) == wasm.Cell.Dead){
            ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
        }
    }
    ctx.stroke()
};

const size = document.getElementById("size");
size.value = width;
const sizeButton = document.getElementById("resize");
sizeButton.addEventListener("click", event => {
    if(size.value > 100){
        size.value = 100;
    }
    if(size.value < 10){
        size.value = 10;
    }
    width = size.value;
    height = size.value;
    universe.set_width(width);
    universe.set_height(height);
    canvas.height = (CELL_SIZE + 1) * height + 1;
    canvas.width = (CELL_SIZE + 1) * width + 1;
    drawCells();
});

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    if(event.ctrlKey){
        if(event.shiftKey){
            universe.toggle_cell(row, col);
            universe.toggle_cell(row - 1, col);
            universe.toggle_cell(row - 2, col);
            universe.toggle_cell(row, col + 1); 
            universe.toggle_cell(row - 1, col + 2);

        }else{
            universe.toggle_cell(row, col);
            universe.toggle_cell(row - 1, col);
            universe.toggle_cell(row - 2, col);
            universe.toggle_cell(row, col - 1);
            universe.toggle_cell(row - 1, col -2);
        }
    }else if(event.shiftKey){
            for(let j = 2; j <= 4; j++){ // starts at 2 only for convenience at the graph, because it starts 2 blocks away from the one clicked.
                universe.toggle_cell(row + 1, col - j);
                universe.toggle_cell(row - 1, col - j);
                universe.toggle_cell(row + 1, col + j);
                universe.toggle_cell(row - 1, col + j);

                universe.toggle_cell(row + j, col - 1);
                universe.toggle_cell(row - j, col - 1);
                universe.toggle_cell(row + j, col + 1);
                universe.toggle_cell(row - j, col + 1);

                universe.toggle_cell(row + j, col - 6);
                universe.toggle_cell(row - j, col - 6);
                universe.toggle_cell(row + j, col + 6);
                universe.toggle_cell(row - j, col + 6);

                universe.toggle_cell(row + 6, col - j);
                universe.toggle_cell(row - 6, col - j);
                universe.toggle_cell(row + 6, col + j);
                universe.toggle_cell(row - 6, col + j);
            }
    }else{
        universe.toggle_cell(row, col);
    }
    drawCells();
});

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸️";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶️";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if(isPaused()){
        play();
    }else{
        pause();
    }
});


let animationId = null;

const isPaused = () => {
    return animationId == null;
};

let count = 0
const speedInput = document.getElementById("speed");
speedInput.addEventListener("change", event => {
    count = 0;
});

let generation = 0;
let generationDisplay = document.getElementById("generation");
generationDisplay.textContent = generation;

const randomButton = document.getElementById("random");
randomButton.addEventListener("click", event => {
    universe.random();
    generation = 0;
    generationDisplay.textContent = generation;
    drawCells();
});

const clearButton = document.getElementById("clear");
clearButton.addEventListener("click", event => {
    universe.clear();
    generation = 0;
    generationDisplay.textContent = generation;
    pause();
    drawCells();
});

const fps = new class{
    constructor(){
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render(){
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        this.frames.push(fps);
        if(this.frames.length > 100){
            this.frames.shift();
        }

        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for(let i = 0; i < this.frames.length; i++){
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max)
        }
        
        let mean = sum / this.frames.length;

        this.fps.textContent = `
        Frames per second:
        latest = ${Math.round(fps)}
        avg of last 100 = ${Math.round(mean)}
        min of last 100 = ${Math.round(min)}
        max of last 100 = ${Math.round(max)}`.trim();
    }
};

const renderLoop = () => {
    debugger;
    fps.render();
    if (count == 51 - speedInput.value){
        universe.tick();
        generation++;
        generationDisplay.textContent = generation;
        count = 0;
    }
    drawCells();
    count++;
    
    animationId = requestAnimationFrame(renderLoop);
};

//decided to remove Draw Grid, it looks better this way
drawCells();
pause();
debugger;
