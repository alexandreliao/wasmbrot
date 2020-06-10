"use strict";

window.onload = function() {
	const canvas = document.getElementById('fractal');
	window.onresize = function() {
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
	};
	window.onresize();
	const context = canvas.getContext('2d');

	let workers = [new Worker(`worker.js?random=${Math.random()}`, type: 'module')];
};
