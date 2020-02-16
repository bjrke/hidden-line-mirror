export default function registerMouse(svg) {

    let origin;

    function onPointerDown(event) {
        origin = {
            ratio: Math.max(viewBox.width / svg.getBoundingClientRect().width, viewBox.height / svg.getBoundingClientRect().height),
            ...getPointFromEvent(event)
        }
    }

    let viewBox = {
        x: -1000,
        y: -1000,
        width: 2000,
        height: 2000
    };

    let newViewBox = null;

    setViewBox(viewBox);

    function onPointerMove(event) {
        if (origin) {
            event.preventDefault();

            const { x, y } = getPointFromEvent(event);

            newViewBox = { ...viewBox };
            newViewBox.x -= (x - origin.x) * origin.ratio;
            newViewBox.y -= (y - origin.y) * origin.ratio;

            setViewBox(newViewBox);
        }
    }

    function setViewBox(vb) {
        svg.setAttribute("viewBox", `${vb.x} ${vb.y} ${vb.width} ${vb.height}`);
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

}
