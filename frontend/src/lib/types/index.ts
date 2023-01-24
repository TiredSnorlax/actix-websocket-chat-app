export interface IInfoMessage {
	info_type: string;
	body: string;
	room_id: string;
}

export interface IUserMessage {
	user_id: string;
	msg: string;
	room_id: string;
}

export interface IRoom {
	name: string;
	description: string;
	id: string;
}
