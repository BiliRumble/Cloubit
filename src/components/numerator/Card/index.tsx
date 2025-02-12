import LazyImage from '../../atoms/LazyImage';
import styles from './Card.module.scss';

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
		<div className={styles.card + ' ' + className} style={style} onClick={onClick}>
			<LazyImage
				src={cover}
				sizes="100%, 100%"
				alt="图片"
				maxRetries={5}
				className={styles.img}
			/>
			<div className={styles.card__content}>
				<h3 className={styles.card__content__title} title={text}>
					{text}
				</h3>
			</div>
		</div>
	);
};

export default Card;
