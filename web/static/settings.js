function updateUser(form) {
    let errorsNode = document.getElementById('settings-errors');
    renderErrors(errorsNode, {});

    let submit = fetchForm(form, { method: 'PUT' });
    let getBody = () => ({
        user: {
            bio: getFormFieldValue(form, 'bio') || undefined,
            image: getFormFieldValue(form, 'image') || undefined,
            password: getFormFieldValue(form, 'password') || undefined,
            username: getFormFieldValue(form, 'username'),
            email: getFormFieldValue(form, 'email'),
        },
    });

    submit(getBody)
        .then((resp) => {
            form.elements.password.value = '';
            window.location.reload();
        })
        .catch((err) => {
            err.res.json().then((json) => {
                renderErrors(errorsNode, json.errors);
            });
        });
}
