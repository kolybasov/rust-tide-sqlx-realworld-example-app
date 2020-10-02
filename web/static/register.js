function register(form) {
    let errorsNode = document.getElementById('register-errors');
    renderErrors(errorsNode, {});

    let submit = fetchForm(form);
    let getBody = () => ({
        user: {
            username: form.elements.username.value,
            email: form.elements.email.value,
            password: form.elements.password.value,
        },
    });

    submit(getBody)
        .then(() => {
            window.location.href = '/';
        })
        .catch((err) => {
            form.elements.password.value = '';
            err.res.json().then((json) => {
                renderErrors(errorsNode, json.errors || { error: [json.message] });
            });
        });
}
