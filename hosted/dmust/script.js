function create_words() {
    let word = document.getElementById("control").value;

    // Create a request to datamuse.com and get the top 10 words
    let request = new XMLHttpRequest();
    request.open("GET", "https://api.datamuse.com/words?ml=" + word + "&max=10", true);
    request.onload = function () {
        // Begin accessing JSON data here
        let data = JSON.parse(this.response);
        if (request.status >= 200 && request.status < 400) {
            let words = document.getElementById("show-list");
            while (words.firstChild) {
                words.removeChild(words.firstChild);
            }
            data.forEach(word => {
                let div = document.createElement("div");
                div.className = "list-group-item";

                div.innerHTML = word.word;
                document.getElementById("show-list").appendChild(div);
            });
        } else {
            console.log("error");
        }
    }
    request.send();
}

document.getElementById("submit").addEventListener("click", () => {
    create_words();
});
