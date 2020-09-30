function login(form) {
    let errorsNode = document.getElementById('login-errors');
    renderErrors(errorsNode, {});

    let submit = fetchForm(form);
    let getBody = () => ({
        user: {
            email: form.elements.email.value,
            password: form.elements.password.value,
        },
    });

    submit(getBody)
        .then(() => {
            window.location.href = '/';
        })
        .catch((err) => {
            err.res.json().then((json) => {
                renderErrors(errorsNode, { error: [json.message] });
            });
        });
}
