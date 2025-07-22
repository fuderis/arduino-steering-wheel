export function js_new_form(selector_or_elem) { return new Form(selector_or_elem); }
export function js_new_forms(selector_or_elem) { return new Forms(selector_or_elem); }

// Form controller
class Form {
    constructor(form) {
        // selector:
        if (typeof form === "string") {
            this.form = document.querySelector(form);
            if (!this.form) throw new Error("Form not found");
        }
        // element:
        else if (form instanceof Element) {
            this.form = form;
        }
        // error:
        else {
            throw new Error("Form constructor expects a selector string or a DOM element");
        }
    }

    // Get field by name
    get_field(name) {
        let fields = this.form.querySelectorAll(`[name="${name}"]`);
        if (!fields.length) return undefined;

        let type = fields[0].type;

        // radio:
        if (type === "radio") {
            let checked = this.form.querySelector(`[name="${name}"]:checked`);
            return checked ? checked.value : null;
        }
        // checkbox:
        else if (type === "checkbox") {
            if (fields.length > 1) {
                return Array.from(fields)
                    .filter(f => f.checked)
                    .map(f => f.value);
            }
            return fields[0].checked;
        }
        // other:
        else {
            return fields[0].value;
        }
    }

    // Serialize form to json
    serialize() {
        let data = {};

        this.form.querySelectorAll("[name]").forEach(el => {
            let name = el.name;
            if (data.hasOwnProperty(name)) return;

            if (el.type === "checkbox") {
                if (!data[name]) data[name] = [];
                if (el.checked) data[name].push(el.value);
            }
            else if (el.type === "radio") {
                if (el.checked) {
                    data[el.name] = el.value;
                }
            }
            else {
                data[el.name] = el.value;
            }
        });

        return data;
    }
}

// Forms delegator
class Forms {
    timers = new Map();
    delay = 1000;

    constructor(delegator) {
        // selector:
        if (typeof delegator === "string") {
            this.delegator = document.querySelector(delegator);
            if (!this.delegator) throw new Error("Forms delegator not found");
        }
        // element:
        else if (delegator instanceof Element) {
            this.delegator = delegator;
        }
        // error:
        else {
            throw new Error("Forms delegator constructor expects a selector string or a DOM element");
        }
    }

    // Set 'input' event handler
    on_input(handler) {
        this.delegator.addEventListener("input", (event) => {
            let input = event.target;
            let form = new Form(input.closest("form"));
            let target_id = form.form.getAttribute("target");

            // reset form timer:
            if (this.timers.has(target_id)) {
                clearTimeout(this.timers.get(target_id));
            }

            // start form timer:
            this.timers.set(target_id, setTimeout(async () => {
                handler(form, input, target_id);
            }, this.delay));
        });

        return this;
    }

    // Set 'submit' event handler
    on_submit(handler) {
        this.delegator.addEventListener("submit", (event) => {
            event.preventDefault();
            let form = new Form(event.target);
            let target_id = form.form.getAttribute("target");

            handler(form, target_id);
        });
        
        return this;
    }
}
