import { createContext } from 'react';
import { eventBus } from '../utils/EventBus';

const EventBusContext = createContext(eventBus);

export const EventBusProvider = ({ children }: { children: React.ReactNode }) => (
	<EventBusContext.Provider value={eventBus}>{children}</EventBusContext.Provider>
);
