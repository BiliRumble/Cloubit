interface TimeProps {
	time: number;
}

const Time: React.FC<TimeProps> = ({ time }) => {
	return <>{new Date(time).toLocaleString()}</>;
};

export default Time;
