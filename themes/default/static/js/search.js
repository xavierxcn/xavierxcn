// Client-side search functionality
let searchIndex = [];
let basePath = '';

function initSearch(path) {
    basePath = path;
    loadSearchIndex();
    setupSearchListeners();
}

async function loadSearchIndex() {
    try {
        const response = await fetch(`${basePath}/search-index.json`);
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }
        searchIndex = await response.json();
        console.log('Search index loaded:', searchIndex.length, 'items');
    } catch (error) {
        console.error('Failed to load search index:', error);
        // Fallback: try without basePath
        try {
            const response = await fetch('/search-index.json');
            if (response.ok) {
                searchIndex = await response.json();
                console.log('Search index loaded (fallback):', searchIndex.length, 'items');
            }
        } catch (e) {
            console.error('Fallback also failed:', e);
        }
    }
}

function setupSearchListeners() {
    const input = document.getElementById('search-input');
    const results = document.getElementById('search-results');

    if (!input || !results) return;

    // Search on input
    input.addEventListener('input', function() {
        const query = this.value.trim();
        if (query.length < 2) {
            results.innerHTML = '';
            results.classList.remove('active');
            return;
        }
        const matches = search(query);
        displayResults(matches, query);
    });

    // Close results when clicking outside
    document.addEventListener('click', function(e) {
        if (!e.target.closest('.search-container')) {
            results.innerHTML = '';
            results.classList.remove('active');
        }
    });

    // Handle keyboard navigation
    input.addEventListener('keydown', function(e) {
        if (e.key === 'Escape') {
            results.innerHTML = '';
            results.classList.remove('active');
            input.blur();
        }
    });
}

function search(query) {
    const queryLower = query.toLowerCase();
    const words = queryLower.split(/\s+/).filter(w => w.length > 0);

    return searchIndex
        .map(item => {
            let score = 0;
            const titleLower = item.title.toLowerCase();
            const summaryLower = item.summary.toLowerCase();
            const contentLower = (item.content || '').toLowerCase();
            const tagsLower = item.tags.map(t => t.toLowerCase());

            // Check each word
            for (const word of words) {
                // Title match (highest priority)
                if (titleLower.includes(word)) {
                    score += 10;
                    // Exact title match bonus
                    if (titleLower === queryLower) {
                        score += 20;
                    }
                }

                // Tag match (high priority)
                if (tagsLower.some(tag => tag.includes(word))) {
                    score += 5;
                }

                // Summary match
                if (summaryLower.includes(word)) {
                    score += 3;
                }

                // Content match
                if (contentLower.includes(word)) {
                    score += 1;
                }
            }

            return { item, score };
        })
        .filter(result => result.score > 0)
        .sort((a, b) => b.score - a.score)
        .slice(0, 10)
        .map(result => result.item);
}

function displayResults(matches, query) {
    const results = document.getElementById('search-results');

    if (matches.length === 0) {
        results.innerHTML = '<div class="search-no-results">未找到相关文章</div>';
        results.classList.add('active');
        return;
    }

    const html = matches.map(item => {
        const highlightedTitle = highlightText(item.title, query);
        const highlightedSummary = highlightText(item.summary, query);

        return `
            <a href="${basePath}${item.url}" class="search-result-item">
                <div class="search-result-title">${highlightedTitle}</div>
                <div class="search-result-meta">
                    <span class="search-result-date">${item.date}</span>
                    ${item.tags.length > 0 ? `<span class="search-result-tags">${item.tags.join(', ')}</span>` : ''}
                </div>
                <div class="search-result-summary">${highlightedSummary}</div>
            </a>
        `;
    }).join('');

    results.innerHTML = html;
    results.classList.add('active');
}

function highlightText(text, query) {
    if (!query) return escapeHtml(text);

    const words = query.toLowerCase().split(/\s+/).filter(w => w.length > 0);
    let result = escapeHtml(text);

    for (const word of words) {
        const regex = new RegExp(`(${escapeRegex(word)})`, 'gi');
        result = result.replace(regex, '<mark>$1</mark>');
    }

    return result;
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function escapeRegex(string) {
    return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
