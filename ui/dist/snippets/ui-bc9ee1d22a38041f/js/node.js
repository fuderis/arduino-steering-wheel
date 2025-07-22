// Creates a new instance of Node
export function js_new_node(elem) { return new Node(elem); }

// Check attribute for exists
export function js_has_attr(node, name) {
    return node.elem.getAttribute(name) !== undefined;
}
// Get attribute value
export function js_get_attr(node, name) {
    return node.elem.getAttribute(name);
}
// Set attribute value
export function js_set_attr(node, name, value) {
    if (value === undefined) {
        node.elem.setAttribute(name, "")
    } else {
        node.elem.setAttribute(name, value)
    }
    return this;
}
// Remove attribute
export function js_remove_attr(node, name) {
    node.elem.removeAttribute(name)
    return this;
}

// Get ID
export function js_get_id(node) {
    return node.elem.getAttribute("id");
}
// Set ID
export function js_set_id(node, name) {
    node.elem.setAttribute("id", name);
    return this;
}
// Remove ID
export function js_remove_id(node) {
    node.elem.removeAttribute("id");
    return this;
}

// Get class list
export function js_get_class_list(node) {
    return node.elem.classList;
}
// Check class name for exists
export function js_has_class(node, name) {
    return node.elem.classList.contains(name);
}
// Add class name
export function js_set_class(node, name) {
    node.elem.classList.add(name);
    return this;
}
// Remove class name
export function js_remove_class(node, name) {
    node.elem.classList.remove(name);
    return this;
}

// Get text contents
export function js_get_text(node) {
    return node.elem.textContent;
}
// Set text contents
export function js_set_text(node, value) {
    node.elem.textContent = value;
    return this;
}

// Get inner html value
export function js_get_html(node) {
    return node.elem.innerHTML;
}
// Set inner html value
export function js_set_html(node, value) {
    node.elem.innerHTML = value;
    return this;
}
// Get outer html value
export function js_get_outer_html(node) {
    return node.elem.outerHTML;
}
// Set outer html value
export function js_set_outer_html(node, value) {
    node.elem.outerHTML = value;
    return this;
}

// Set event handler
export function js_set_event(node, name, handler) {
    node.elem.addEventListener(name, handler);
    return this;
}

// Insert node
export function js_insert_to(node, index, child) {
    let elem = child instanceof Node ? node.elem : child;
    if (!(elem instanceof Element)) {
        throw new Error("The node must be an instance of a Node or a DOM element.");
    }

    // get children nodes:
    let children = Array.from(node.elem.children);
    let len = children.length;

    // gen index:
    if (index < 0) {
        index = len + index;
        if (index < 0) index = 0;
    }
    if (index > len) index = len;

    // insert elem:
    if (index === len) {
        node.elem.appendChild(elem);
    } else {
        node.elem.insertBefore(elem, children[index]);
    }

    return this;
}
// Insert node to front
export function js_insert_front(node, child) {
    return node.insert(0, child);
}
// Insert node to back
export function js_insert_back(node, child) {
    return node.insert(-1, child);
}

// The DOM element controller
class Node {
    elem = undefined;

    constructor(elem) {
        // html tag:
        if (typeof elem === "string" && elem.trim().startsWith("<")) {
            let template = document.createElement("template");
            template.innerHTML = elem.trim();

            this.elem = template.content.firstElementChild;
            if (!this.elem) throw new Error("Failed to create a new element from HTML");
        }
        // selector:
        else if (typeof elem === "string") {
            this.elem = document.querySelector(elem);
            if (!this.elem) throw new Error("Element not found");
        }
        // element:
        else if (elem instanceof Element) {
            this.elem = elem;
        }
        // error:
        else {
            throw new Error("Element constructor expects a selector string or a DOM element");
        }
    }
}
