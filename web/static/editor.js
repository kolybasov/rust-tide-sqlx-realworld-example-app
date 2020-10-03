function createOrUpdateArticle(form, options) {
    let errorsNode = document.getElementById('editor-errors');
    renderErrors(errorsNode, {});

    let submit = fetchForm(form, options);
    let getBody = () => ({
        article: {
            title: form.elements.title.value,
            description: form.elements.description.value,
            body: form.elements.body.value,
            tagList: form.elements.tags.value
                .split(',')
                .map((tag) => tag.trim().toLowerCase())
                .filter(Boolean),
        },
    });

    submit(getBody)
        .then((resp) => {
            window.location.href = `/article/${resp.article.slug}`;
        })
        .catch((err) => {
            err.res.json().then((json) => {
                renderErrors(errorsNode, json.errors);
            });
        });
}
