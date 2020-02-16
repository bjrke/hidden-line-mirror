export default function registerMouse(svg) {

    let origin;

    function onPointerDown(event) {
        const { width, height } = svg.getBoundingClientRect();
        origin = {
            rectSize: Math.min(width, height),
            ...getPointFromEvent(event)
        }
    }

    let viewBox = {
        x: -1000,
        y: -1000,
        size: 2000
    };

    let newViewBox = null;

    setViewBox(viewBox);

    function onPointerMove(event) {
        if (origin) {
            event.preventDefault();

            const { x, y } = getPointFromEvent(event);

            newViewBox = { ...viewBox };
            newViewBox.x -= (x - origin.x) * viewBox.size / origin.rectSize;
            newViewBox.y -= (y - origin.y) * viewBox.size / origin.rectSize;

            setViewBox(newViewBox);
        }
    }

    function setViewBox(vb) {
        svg.setAttribute("viewBox", `${vb.x} ${vb.y} ${vb.size} ${vb.size}`);
    }

    function onPointerUp() {
        origin = null;
        if (newViewBox) {
            viewBox = newViewBox;
            newViewBox = null;
        }
    }

    if (window.PointerEvent) {
        document.addEventListener("pointerdown", onPointerDown);
        document.addEventListener("pointerup", onPointerUp);
        document.addEventListener("pointerleave", onPointerUp);
        document.addEventListener("pointermove", onPointerMove);
    } else {
        document.addEventListener("mousedown", onPointerDown);
        document.addEventListener("mouseup", onPointerUp);
        document.addEventListener("mouseleave", onPointerUp);
        document.addEventListener("mousemove", onPointerMove);

        document.addEventListener("touchstart", onPointerDown);
        document.addEventListener("touchend", onPointerUp);
        document.addEventListener("touchmove", onPointerMove);
    }

    function getPointFromEvent(event) {
        const p = event.targetTouches ? event.targetTouches[0] : event;
        return {
            x: p.clientX,
            y: p.clientY
        };
    }

    function onWheel(event) {
        event.preventDefault();

        const additionalSize = viewBox.size * event.deltaY / 100;
        const rect = svg.getBoundingClientRect();
        const rectSize = Math.min(rect.width, rect.height);
        const p = getPointFromEvent(event);

        const f = additionalSize / (2 * rectSize);
        viewBox.x -= (2 * p.x + rectSize - rect.width) * f;
        viewBox.y -= (2 * p.y + rectSize - rect.height) * f;
        viewBox.size += additionalSize;

        setViewBox(viewBox);
    }

    document.addEventListener("wheel", onWheel);
}
