import LazyImage from '../LazyImage';
import styles from './Card.module.scss';

interface CardProps {
	key?: string | number;
	cover: string;
	text: string;
	style?: React.CSSProperties;
	onClick?: () => void;
	className?: string;
}

const Card: React.FC<CardProps> = ({
	key,
	className = '',
	style = {},
	cover,
	text,
	onClick = () => {},
}) => {
	return (
		<div key={key} className={styles.card + ' ' + className} style={style} onClick={onClick}>
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
