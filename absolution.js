function absolution() {
    for (const e of document.getElementsByClassName("abspos")) {
        console.log(`found element ${JSON.stringify(e.classList)}`);
        var top = NaN;
        var left = NaN;
        for (const c of e.classList) {
            if (c.startsWith("top")) {
                top = Number(c.substring(3));
            }

            if (c.startsWith("left")) {
                left = Number(c.substring(4));
            }
        }

        e.style.position = "absolute";

        if (top === top) {
            console.log(`found top ${top}`);
            e.style.top = `${top}px`;
        }

        if (left === left) {
            console.log(`found left ${left}`);
            e.style.left = `${left}px`;
        }
    }
}