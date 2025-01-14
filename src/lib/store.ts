import { writable } from 'svelte/store';

import type { ChatMessage } from './types/message.type';

// https://sveltebyexample.com/writable-stores/

// 聊天类型，1= 共情， 2= 低共情
export const chatType = writable<string>('')

export const chatMessages = writable<Array<ChatMessage>>([]);

export const defaultMessage = writable<ChatMessage>();