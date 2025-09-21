export const init = onActivate => {
    if (Date.now() < new Date('2025-10-04T08:00:00.000Z') && location.search !== '?debug=1') return;

    let dragged;
    let count;

    const c1 = document.getElementById('c1');
    const c2 = document.getElementById('c2');
    const c3 = document.getElementById('c3');
    const steam = document.getElementById('steam');
    const img = new Image();
    img.src = './assets/ui/coin.png';
    const coin = new Audio('./assets/ost/coin.ogg');

    const onMouseEnter = event => {
        event.target.draggable = true;
    };

    const onMouseLeave = event => {
        event.target.removeAttribute('draggable');
    };

    const onDragStart = event => {
        event.dataTransfer.setDragImage(img, 0, 0);
        dragged = event.target;
    };

    const onDragOver = event => {
        event.preventDefault();
    };

    const spawnBonus = () => {
        const bonus = document.createElement('button');
        bonus.id = 'bonus';
        bonus.style.top = `${Math.random() * 100}%`;

        document.body.appendChild(bonus);

        let activated = false;

        bonus.addEventListener('click', () => {
            activated = true;
            onActivate();
            bonus.remove();
            start();
        });
        bonus.addEventListener('animationend', () => {
            if (activated) return;
            bonus.remove();
            start();
        });
    };

    const onDrop = event => {
        event.preventDefault();

        if (!dragged) return;

        dragged.removeEventListener('mouseenter', onMouseEnter);
        dragged.removeEventListener('mouseleave', onMouseLeave);
        dragged.removeAttribute('draggable');

        coin.cloneNode().play();
        count++;

        dragged = undefined;

        if (count === 3) {
            destroy();
            spawnBonus();
        }
    };

    const start = () => {
        count = 0;
        dragged = undefined;

        [c1, c2, c3].forEach(c => {
            c.addEventListener('mouseenter', onMouseEnter);
            c.addEventListener('mouseleave', onMouseLeave);
            c.addEventListener('dragstart', onDragStart);
        });
        steam.addEventListener('dragover', onDragOver);
        steam.addEventListener('drop', onDrop);
    };

    const destroy = () => {
        [c1, c2, c3].forEach(c => {
            c.removeAttribute('draggable');
            c.removeEventListener('dragstart', onDragStart);
            c.removeEventListener('mouseenter', onMouseEnter);
            c.removeEventListener('mouseleave', onMouseLeave);
        });
        steam.removeEventListener('dragover', onDragOver);
        steam.removeEventListener('drop', onDrop);
    };

    start();
};
