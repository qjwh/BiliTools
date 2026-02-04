import { MessageRenderMessage, MessageType, NAlert } from "naive-ui";
import { MessageApiInjection } from "naive-ui/es/message/src/MessageProvider";
import { h } from "vue";

let injection: MessageApiInjection | null = null;

export function injectMessage(source: MessageApiInjection) {
  injection = source;
}

interface CustomContent {
  title: string,
  body?: string,
}

const renderMessage: MessageRenderMessage = (props) => {
  if (typeof props.content !== 'string') return;
  const content = JSON.parse(props.content) as CustomContent;

  return h(
    NAlert,
    {
      closable: props.closable,
      onClose: props.onClose,
      type: props.type === 'loading' ? 'default' : props.type,
      title: content.title,
    },
    {
      default: content.body ? () => content.body : undefined,
    }
  )
}

export function notify(opt: {
  title: string,
  body?: string,
  type?: MessageType
}) {
  const content: CustomContent = {
    title: opt.title,
    body: opt.body,
  }
  injection?.create(JSON.stringify(content), {
    type: opt.type,
    closable: true,
    duration: opt.type === 'error' ? 5000 : 2500,
    render: renderMessage
  }
  )
}
