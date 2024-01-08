function randomChar() {
	const characters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
	return characters.charAt(Math.floor(Math.random() * characters.length));
}

function randomClassName(length = 5) {
	let result = '';
	for (let i = 0; i < length; i++) {
		result += randomChar();
	}
	return result;
}

window.onload = function () {
	let hiddenClass = randomClassName();
	let invisibleClass = randomClassName();
	let neutralClass = randomClassName();

	let style = document.createElement('style');

	style.innerHTML = `
	  .${hiddenClass} {
		display: none;
	  }
	  
	  .${invisibleClass} {
		display: inline-block;
		visibility: hidden;
		width: 0;
	  }
	  
	  .${neutralClass} {
		box-sizing: border-box;
	  }
	  `;

	document.head.appendChild(style);

	setTimeout(function () {
		for (let item of document.getElementsByClassName("obfuscated")) {
			let unencoded = atob(item.innerText.split('').reverse().join(''));
			item.innerHTML = '';

			for (let char of unencoded) {
				let span = document.createElement('span');
				span.innerText = char;
				span.className = neutralClass;
				item.appendChild(span);

				let run = 0;

				do {
					let hiddenSpan = document.createElement('span');
					hiddenSpan.innerText = randomChar();
					hiddenSpan.className = Math.random() < 0.5 ? hiddenClass : invisibleClass;
					item.appendChild(hiddenSpan);
					run++;
				} while (Math.random() < 0.5 && run < 4);
			}

			item.classList.remove("obfuscated");
		}

		for (let item of document.getElementsByClassName("obfuscation-note")) {
			item.remove();
		}
	}, 1000);
}
