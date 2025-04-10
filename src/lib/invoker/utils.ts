export enum INVOKERS {
    GET_CONTAINERS = 'get_containers',
    GET_CONTAINER = 'get_container_by_id',
    INSPECT_CONTAINER = 'fetch_container_info',
    CHECK_PREREQUISITES = 'check_prerequisites',
    START_COLIMA_SERVICE = 'start_colima_service',
    STOP_COLIMA_SERVICE = 'stop_colima_service',
    CONNECT_TO_DOCKER = 'connect_to_docker',
    GET_IMAGES = 'get_images',
    GET_IMAGE_INFO = 'get_image_info',
    GET_IMAGE_HISTORY = 'get_image_history',
    DELETE_IMAGE = 'delete_image',
}