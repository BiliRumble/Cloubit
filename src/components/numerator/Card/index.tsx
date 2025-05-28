import LazyImage from '@/components/atoms/Image';

interface CardProps {
	key?: string | number;
	cover: string;
	text: string;
	style?: React.CSSProperties;
	onClick?: () => void;
	className?: string;
}

/**
 * Card 组件
 *
 * @param {string} key - 唯一标识
 * @param {string} cover - 封面图片
 * @param {string} text - 标题
 * @param {React.CSSProperties} style - 样式
 * @param {() => void} onClick - 点击事件
 * @param {string} className - 类名
 */
const Card: React.FC<CardProps> = ({
	className = '',
	style = {},
	cover,
	text,
	onClick = () => {},
}) => {
	return (
		<div
			className={'flex flex-col items-start h-full w-full flex-nowrap' + ' ' + className}
			style={style}
			onClick={onClick}
		>
			<LazyImage
				src={cover}
				sizes="100%, 100%"
				alt="图片"
				className="rounded-md w-full! h-auto! aspect-w-16! aspect-h-9!"
			/>
			<div className="flex flex-col items-start pt-0.3rem pb-0.3rem w-full h-full">
				<h3
					className="text-1.2rem max-w-[100%] whitespace-normal text-[var(--second-text-color)] overflow-hidden text-ellipsis box break-all"
					title={text}
				>
					{text}
				</h3>
			</div>
		</div>
	);
};

export default Card;
