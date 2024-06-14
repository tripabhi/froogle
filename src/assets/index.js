async function search(query) {
  const resultsDivElement = document.getElementById("results");
  resultsDivElement.innerHTML = "";

  const response = await fetch("/api/search", {
    method: "POST",
    headers: { "Content-Type": "text/plain" },
    body: query,
  });

  const json = await response.json();
  for ([path, file] of json) {
    let resultDivElement = document.createElement("div");
    resultDivElement.className = "result";
    renderResult(resultDivElement, path, file);
    resultsDivElement.appendChild(resultDivElement);
  }
}

function renderResult(root, path, file) {
  let spanTag = document.createElement("span");
  spanTag.className = "fn";
  spanTag.appendChild(document.createTextNode(file));

  let aTag = document.createElement("a");
  aTag.href = path;
  aTag.textContent = path;
  aTag.className = "fa";
  aTag.target = "_blank";
  aTag.rel = "noopener noreferrer";

  root.appendChild(spanTag);
  root.appendChild(aTag);
}

let query = document.getElementById("query");
let searchPromise = Promise.resolve();

query.addEventListener("keypress", (e) => {
  if (e.key == "Enter") {
    searchPromise.then(() => search(query.value));
  }
});
