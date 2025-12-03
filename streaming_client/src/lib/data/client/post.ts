
export const updateAnimeBackgroundPosition = async (id: string, backgroundPosition: number) => {
    let formData = new FormData();
    formData.append('id', id);
    formData.append('background_position', backgroundPosition.toString()); //credentials include
    const response = await fetch(`/server/update_anime_background_position`, {
        method: 'POST',
        body: formData,
        credentials: 'include'
    });
    return response.json();
}