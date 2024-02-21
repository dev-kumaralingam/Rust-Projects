//frontend/searchbar.js
import { search } from './searchbar_bg.js';

async function doSearch() {
    let searchInput = document.getElementById("searchInput").value.trim();
    let searchResultsElement = document.getElementById("searchResults");

    if (searchInput.length === 0) {
        searchResultsElement.innerHTML = ""; // Clear previous search results
        return;
    }

    try {
        const results = await search(searchInput, 5);
        renderResults(results);
    } catch (error) {
        console.error("Error while searching:", error);
    }
}

function renderResults(results) {
    let searchResultsElement = document.getElementById("searchResults");
    searchResultsElement.innerHTML = ""; // Clear previous search results

    results.forEach(([title, url]) => {
        let listItem = document.createElement("li");
        let link = document.createElement("a");
        link.href = url;
        link.textContent = title;
        listItem.appendChild(link);
        searchResultsElement.appendChild(listItem);
    });
}
