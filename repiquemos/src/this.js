const states = ["unknown", "suspended", "active", "complete", "closed", "settled"];

function maxState(...xs) {
    const indexes = xs.map(states.indexOf);
    return states[Math.max(...indexes)];
}

function maxStateBound(...xs) {
    const indexes = xs.map(states.indexOf.bind(states));
    return states[Math.max(...indexes)];
}

console.log(maxStateBound("unknown", "suspended", "active"));