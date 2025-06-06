function applyScale() {
	const baseWidth = 800;
	const scale = Math.min(window.innerWidth / baseWidth, 1);
	const inner = document.querySelector('.scale-inner');
	inner.style.transform = `scale(${scale})`;
}

window.addEventListener('resize', applyScale);
window.addEventListener('DOMContentLoaded', applyScale);
