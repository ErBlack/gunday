let statusSection;
let codeButton;
let timeout;


export const eggStatus = startGame => {
    if (Date.now() < new Date('2025-10-04T08:00:00.000Z') && location.search !== '?debug=1') return;

    const status = localStorage.getItem('gundayStatus');

    if (!status) return;

    if (!statusSection) {
        statusSection = document.createElement('section');

        const data = document.getElementById('date');

        data.parentNode.insertBefore(statusSection, data.nextSibling);

        const start = document.createElement('button');
        start.textContent = 'start';
        start.onclick = startGame;
        

        
        codeButton = document.createElement('button');
        codeButton.id = 'code';

        codeButton.onclick = (event) => {
            clearTimeout(timeout);
            navigator.clipboard.writeText(localStorage.getItem('gundayCode'));

            codeButton.textContent = 'code copied!';
            
            timeout = setTimeout(() => {
                codeButton.textContent = localStorage.getItem('gundayCode') || '';
            }, 2000);
        }

        statusSection.appendChild(start);
        statusSection.appendChild(codeButton);
    }

    codeButton.textContent = localStorage.getItem('gundayCode') || '';
};
