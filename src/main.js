const { invoke } = window.__TAURI__.tauri;

async function write_to_whazzup() {
	let callsigin = document.querySelector("#call").value;
	let name = document.querySelector("#name").value;
	let lat = document.querySelector("#lat").value;
	let lon = document.querySelector("#lon").value;
	let alt = document.querySelector("#alt").value;
	let spd = document.querySelector("#spd").value;
	let type = document.querySelector("#type").value;
	let squawk = document.querySelector("#squawk").value;
	let rule = document.querySelector("#rule").value;
	let route = document.querySelector("#route").value;
	let hdg = document.querySelector("#hdg").value;
	document.querySelector("#result").innerHTML = await invoke("create_whazzup_txt", {call: callsigin, name: name, lat: lat, lon: lon, alt: alt, spd: spd, actype: type, squawk: squawk, rule: rule, route:route, hdg: hdg});
}

window.addEventListener("DOMContentLoaded", () => {
	document
		.querySelector("#write_button")
		.addEventListener("click", () => write_to_whazzup());
});