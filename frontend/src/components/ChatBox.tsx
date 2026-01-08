import { useEffect } from 'react';
import {createBot} from 'botui';
import {BotUI, BotUIMessageList, BotUIAction, useBotUI, useBotUIAction} from "@botui/react"
import '../assets/default.theme.scss'


const myBot = createBot()

const StarsAction = () => {
    const bot = useBotUI();
    const action = useBotUIAction();
    const array = new Array(action?.data.total).fill('⭐️');

    return (
        <div>
            {array.map((v, i) => (
                <button
                    key={i}
                    onClick={() => {
                        bot.next({ starsGiven: i + 1 }, { messageType: 'stars' });
                    }}
                >
                    {/*@ts-ignore*/}
                    {i + 1} {v}
                </button>
            ))}
        </div>
    );
};

interface Message {
    data: {
        starsGiven: number
    }
}

const StarsMessage = ({ message }: { message: Message }) => {
    const stars = new Array(message.data.starsGiven).fill('⭐️')

    return (
        <div>
            {stars}
        </div>
    )
}

const actionRenderers = {
    'stars': StarsAction
}

const messageRenderers = {
    'stars': StarsMessage
}

function ChatBox() {
    useEffect(() => {
        // @ts-ignore
        myBot.message.add({
            text: "你好，这里是 Saudade Blog👋",
        }).then(() => {
            return myBot.wait({ waitTime: 1500 });
        }).then(() => {
            return myBot.message.add({
                text: "我是 Sora😄",
            });
        }).then(() => {
            return myBot.wait({ waitTime: 1500 });
        }).then(() => {
            return myBot.message.add({
                text: "是 [ Saudade Blog ] 的维护作者",
            });
        }).then(() => {
            return myBot.wait({ waitTime: 1500 });
        }).then(() => {
            return myBot.action.set(
                {
                    options: [
                        { label: '然后呢？😃', value: 'and' },
                        // { label: '少废话！😆', value: 'gg' },
                    ],
                },
                { actionType: 'selectButtons' }
            );
            //@ts-ignore
        }).then((res: any) => {
            console.log(res);
            if (res.value == "and") {
                return myBot.next()
            }
            if (res.value == "gg") {
                return myBot.message.add({
                    text: "![](https://view.amogu.cn/images/2020/08/30/sanlian.jpg)",
                });
            }
        }).then(async () => {
            await myBot.message.add({
                text: "😘",
            });
            await myBot.wait({waitTime: 1500});
            await myBot.message.add({
                text: "本项目是在[ Memory ]基础上二次开发完成的",
            });
            await myBot.wait({waitTime: 1500});
            await myBot.message.add({
                text: "本项目使用Rust对其后端进行了重构优化与拓展",
            });
            await myBot.wait({waitTime: 1500});
            await myBot.message.add({
                text: "虽然前端好像被我改得更丑了好像。。。。",
            });
            await myBot.wait({waitTime: 1500});
            await myBot.message.add({
                text: "不要再问了诶",
            });
            await myBot.wait({waitTime: 1500});
            await myBot.action.set(
                {
                    options: [
                        {label: '就问，就问，嘻嘻嘻', value: 'why'},
                    ],
                },
                {actionType: 'selectButtons'}
            );
            await myBot.wait({waitTime: 1500});
            await myBot.message.add({
                text: "好吧最后一个问题",
            });
            await myBot.wait({waitTime: 1500});
            await myBot.action.set(
                {
                    options: [
                        {label: '你到底是谁？', value: 'like'},
                    ],
                },
                {actionType: 'selectButtons'}
            );
            await myBot.wait({waitTime: 1500});
            await myBot.message.add({
                text: "我。我是。。。。",
            });
            await myBot.wait({waitTime: 1500});
            await myBot.message.add({
                text: "我是嫩爹！",
            });
        }).then(async () => {
            await myBot.wait({waitTime: 1500});
            return myBot.message.add({
                text: "给 [ Memory ] 评个星吧！"
            }).then(() => {
                myBot.action.set(
                    { total: 6 },
                    { actionType: 'stars' }
                )
                    .then(async (data) => { // data 是从 .next() 返回的数据
                        await myBot.message.add({text: `你对 [ Memory ] 的评价是 ${data.starsGiven} 星!`});
                        await myBot.wait({waitTime: 1500});
                        return myBot.message.add({
                            text: "再见啦，祝你开心呦 ^_^",
                        });
                    });
            })
        });

        return () => {
        //     销毁
            myBot.message.removeAll()
        }
    },[])
    return (
        <div className='chatbot' id='linmo'>
            <div className="logofont" style={{ textAlign: 'center', fontSize: '50px', marginBottom: '20px', marginRight: '-20px' }}>[Saudade Blog]</div>
            <div className="popcontainer" id="fogforest" style={{ minHeight: '500px', padding: '2px 6px 4px 6px', backgroundColor: 'rgba(242, 242, 242,0.5)', borderRadius: '10px',display:'flex',flexDirection:'column'
            ,justifyContent:'center'
            }}>
                <BotUI bot={myBot}>
                    <BotUIMessageList renderer={messageRenderers} />
                    <BotUIAction renderer={actionRenderers} />
                </BotUI>
                <div>
                </div>
            </div>

        </div>
    )
}

export default ChatBox;
