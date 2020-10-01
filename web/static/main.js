function renderErrors(errorsNode, errors) {
    let errorsList = errorsNode.getElementsByTagName('ul')[0];
    if (errorsList) {
        oldList = errorsList;
        errorsList = document.createElement('ul');
        errorsNode.replaceChild(errorsList, oldList);
    } else {
        errorsList = document.createElement('ul');
    }

    let fields = Object.keys(errors);
    if (fields.length === 0) return;

    errorsList.classList.add(
        'rounded-md',
        'border',
        'border-red-600',
        'bg-red-200',
        'p-4',
        'text-red-600',
        'list-disc',
        'list-inside'
    );

    for (let i = 0; i < fields.length; i++) {
        let key = fields[i];
        let fieldErrors = errors[key];

        for (let j = 0; j < fieldErrors.length; j++) {
            let errorMsg = fieldErrors[j];
            let el = document.createElement('li');
            el.classList.add('capitalize');
            el.textContent = `${key} ${errorMsg}`;
            errorsList.appendChild(el);
        }
    }

    errorsNode.appendChild(errorsList);
}

function fetchForm(form, options = {}) {
    return (getBody) => {
        let originalText;
        let submitBtn = form.getElementsByTagName('button')[0];
        if (submitBtn) {
            originalText = submitBtn.textContent;
            submitBtn.disabled = true;
            submitBtn.textContent = 'Loading...';
        }

        return fetch(options.action || form.action, {
            ...options,
            method: options.method || form.method,
            headers: {
                'content-type': 'application/json',
                accept: 'application/json',
            },
            body: getBody && JSON.stringify(getBody(form)),
        }).then((res) => {
            if (submitBtn) {
                submitBtn.disabled = false;
                submitBtn.textContent = originalText;
            }

            if (res.ok) {
                if (res.status === 204) return;
                return res.json();
            } else {
                let err = new Error(res.statusText);
                err.res = res;
                throw err;
            }
        });
    };
}
