document.addEventListener('DOMContentLoaded', () => {
    // Toggle endpoint
    document.querySelectorAll('.endpoint-header').forEach(h => {
        h.addEventListener('click', () => {
            h.parentElement.classList.toggle('open');
        });
    });

    // Toggle nav groups
    document.querySelectorAll('.nav-group-title').forEach(t => {
        t.addEventListener('click', () => {
            const items = t.nextElementSibling;
            if (items) items.classList.toggle('collapsed');
        });
    });

    // Search
    const search = document.getElementById('search');
    if (search) {
        search.addEventListener('input', (e) => {
            const q = e.target.value.toLowerCase().trim();
            document.querySelectorAll('.endpoint').forEach(ep => {
                const text = ep.textContent.toLowerCase();
                ep.classList.toggle('hidden', q && !text.includes(q));
            });
            document.querySelectorAll('.nav-item').forEach(item => {
                const text = item.textContent.toLowerCase();
                item.classList.toggle('hidden', q && !text.includes(q));
            });
            // Expand matching groups
            if (q) {
                document.querySelectorAll('.nav-items').forEach(g => g.classList.remove('collapsed'));
            }
        });
    }

    // Open first endpoint
    const first = document.querySelector('.endpoint');
    if (first) first.classList.add('open');
});
