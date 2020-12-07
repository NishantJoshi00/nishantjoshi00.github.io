var w = $('.thumb').height();
$('.thumb').css('width', w);
let a = null;

const square_function = () => {
	const height = $('.thumb').height();
	$('.thumb').css('width', height);
}


const data_usage = () => {
	const oauth_token = "BQCsul0OwHMmM4S12vlabumKVWWAfWuJtQR_ks8paspFHdbyN7XbdP6_N5Uk1Txuf6i78q4QHyM3Ki1FqqeJPw_P0spaus0Uct1bMz5vSVMeWsbIv-9dHvdKisKfQhJ2ilXcVE-3XPhKSXlJPgUjiS-kKxOQdkz1Q5YsX7jHue1V-A"
	const url = "https://api.spotify.com/v1/me/player"
	var output = null;
	const params = {
		headers: {
			"Content-Type": "application/json",
			"Authorization": `Bearer ${oauth_token}`
		},
		method: "GET"
	}
	fetch(url, params)
	.then(data => {
		data.json()
		.then(jdumb => {
			document.getElementById("song-name").innerText = jdumb.item.name;
			a = jdumb
			document.getElementsByClassName("thumb")[0].style.backgroundImage = `url('${jdumb.item.album.images[0].url}')`
			if (jdumb.is_playing) {
				document.getElementById("status").src = "pause.svg"
			} else {
				document.getElementById("status").src = "play.svg"
			}
		})
	})
}

data_usage()
const eventLoop = () => {
	square_function()
	data_usage()
}

setInterval(eventLoop, 200);
