function postComment(form) {
    let errorsNode = document.getElementById('comment-errors');
    renderErrors(errorsNode, {});

    let submit = fetchForm(form);
    let getBody = () => ({
        comment: {
            body: form.elements.body.value,
        },
    });

    submit(getBody)
        .then(() => {
            form.elements.body.value = '';
            window.location.reload();
        })
        .catch((err) => {
            err.res.json().then((json) => {
                renderErrors(errorsNode, { error: [json.message] });
            });
        });
}
function deleteComment(btn, url) {
    let submit = fetchForm(btn, { action: url, method: 'DELETE' });
    submit().then(() => {
        window.location.reload();
    });
}
