import fs from 'fs';
import path from 'path';
import yaml from 'js-yaml';

export interface Config {
    app: {
        name: string;
        version: string;
        domain: string;
        backend_url: string;
        frontend_url: string;
    };
    group: {
        name: string;
    };
    session: {
        duration_minutes: number;
    };
    debug: boolean;
    surrealdb: {
        host: string;
        port: number;
        user: string;
        password: string;
        namespace: string;
        database: string;
    };
    discord: {
        episodes_webhook_url: string;
    };
    directories: {
        media_dir: string;
        anime_folder: string;
        anime_thumbnails_folder: string;
        episodes_folder: string;
        episodes_images_folder: string;
        users_folder: string;
        torrents_folder: string;
        subtitles_folder: string;
        avatars_folder: string;
        roles_icons_folder: string;
        news_main_folder: string;
        news_images_folder: string;
        news_thumbnails_folder: string;
    };
}
// import config from '../../../../config.yml';

export function loadConfig(): Config {
    const configPath = path.resolve('../config.yml');
    const fileContents = fs.readFileSync(configPath, 'utf8');
    const data = yaml.load(fileContents) as Config;
    // console.log(data);
    return data;
}