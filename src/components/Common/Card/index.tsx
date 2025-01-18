import styles from './Card.module.scss';

interface CardProps {
	cover: string;
	text: string;
	url?: string;
	style?: React.CSSProperties;
	className?: string;
}

const Card: React.FC<CardProps> = ({ className = '', style = {}, cover, text, url }) => {
	return (
		<div className={styles.card + ' ' + className} style={style}>
			<img src={cover} alt="卡片的图片" onClick={() => window.open(url, '_blank')} />
			<div className={styles.card__content}>
				<h3 className={styles.card__content__title}>{text}</h3>
			</div>
		</div>
	);
};

export default Card;
